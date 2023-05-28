import { Component } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {ActivatedRoute, ParamMap, Router} from "@angular/router";
import { UserService } from '../user.service';
import { BastionInfo } from '../bastion-info';
import { RessourceInfo } from '../ressource-info';
import { AuthenticationService } from 'src/app/login/authentication.service';

@Component({
  selector: 'app-ressources',
  templateUrl: './ressources.component.html',
  styleUrls: ['./ressources.component.scss']
})
export class RessourcesComponent {


  public bastion! : BastionInfo;

  public listRessources : Array<RessourceInfo> = new Array<RessourceInfo>();

  public bastion_id : string = '';


  constructor(protected userService : UserService, protected serviceAuthentication: AuthenticationService,     private activRoute: ActivatedRoute) { 

  }

  ngOnInit(): void {


    this.activRoute.paramMap.subscribe((params: ParamMap) => {
      this.bastion_id = params.get('idBastion') || '';

      this.userService.get_a_bastion(this.bastion_id).subscribe({

        next: (data : any) => {
          this.bastion = data.data as BastionInfo 
        }

      });

      this.getListlistRessources()
    });

   
  }

  
  getListlistRessources(){

    this.userService.get_ressources(this.bastion_id).subscribe({

      next: (data : any) => {
        
        this.listRessources = data.data as RessourceInfo[]

        
      },
      error: (e) => {
        
        console.error(e)
      },
    })

  }

}