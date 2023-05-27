import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { UserBastionInfo } from '../user-bastion-info';
import { AdminService } from '../admin.service';
import { UserInfo } from '../user-info';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { ActivatedRoute, ParamMap } from '@angular/router';

@Component({
  selector: 'app-list-user-resource-detail',
  templateUrl: './list-user-resource-detail.component.html',
  styleUrls: ['./list-user-resource-detail.component.scss']
})
export class ListUserResourceDetailComponent {

  @Input() userBastion!: UserBastionInfo;
  @Output() newItemEvent = new EventEmitter<string>();

  public user!: UserInfo;

  public message : string ='';

  public bastion_id: string = '';

  public ressource_id: string = '';


  constructor(protected adminService : AdminService, private modalService: NgbModal, protected activRoute: ActivatedRoute){

    this.activRoute.paramMap.subscribe((params: ParamMap) => {
      this.bastion_id = params.get('idBastion') || '';

      this.activRoute.paramMap.subscribe((params: ParamMap) => {
        this.ressource_id = params.get('idRessource') || '';

      });
    });

    this.getMailUser(this.userBastion)
  }


  openWindowCustomClass(content: any) {

    this.modalService.open(content);
  }


  supprimer(){

    this.adminService.delete_a_user_on_a_ressource(this.bastion_id, this.ressource_id, this.user.id).subscribe({

      next: (data : any) => {
        
        this.message="L'utilisateur a bien été supprimé"
        this.newItemEvent.emit("delete");

        
      },
      error: (e) => {
        
        this.message="Impossible de supprimer l'utilisateur"
      },
    })

  }

  getMailUser(user: UserBastionInfo){

    this.adminService.get_user_mail(user.user_id).subscribe({

        next: (data : UserInfo[]) => {

          if(data.length == 0) return

          else if (data.length == 1){
            this.user = data[0]

          }

        }
      })
    }
  }

