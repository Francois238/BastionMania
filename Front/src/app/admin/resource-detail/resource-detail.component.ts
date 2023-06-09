import { Component, EventEmitter, Input, Output } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { AdminService } from '../admin.service';
import { Router } from '@angular/router';
import { RessourceInfo } from '../ressource-info';

@Component({
  selector: 'app-resource-detail',
  templateUrl: './resource-detail.component.html',
  styleUrls: ['./resource-detail.component.scss']
})
export class ResourceDetailComponent {

  @Input() ressource!: RessourceInfo;
  @Output() newItemEvent = new EventEmitter<string>();

  message : string =''

  constructor(protected adminService : AdminService, private modalService: NgbModal, protected router: Router) { }


  openWindowCustomClass(content: any) {

    this.modalService.open(content);
  }


  supprimer(){

    this.adminService.delete_a_ressource(this.ressource.id_bastion,this.ressource.id).subscribe({

      next: (data : any) => {
        
        this.message="La ressource a bien été supprimée"
        this.newItemEvent.emit("delete");

        
      },
      error: (e) => {
        
        this.message="Impossible de supprimer la ressource"
      },
    })

  }

  getDetail(){

    this.router.navigate([`/admin/bastions/${this.ressource.id_bastion}/${this.ressource.id}`]);
  }


}
